import copy
import sys
import yaml


TEMPLATE_FILE = "docker-compose-dev.yaml.template"
OUTPUT_FILE = "docker-compose-dev.yaml"

def set_n_clients(num):
    template_file = open(TEMPLATE_FILE, 'r')
    template = yaml.load(template_file, Loader=yaml.FullLoader)

    client_template = template["services"]["client"]
    del(template["services"]["client"])
    for i in range(0, num):
        cur_client = copy.deepcopy(client_template)
        client_name = "client" + str(i)
        cur_client["container_name"] = client_name
        cur_client["environment"][0] += str(i)
        template["services"][client_name] = cur_client

    output_file = open(OUTPUT_FILE, 'w')
    dump = yaml.dump(template)
    output_file.write(dump)

if __name__ == "__main__":
    if len(sys.argv) <= 1:
        print ("Usage: {} <n_clients>".format(sys.argv[0]))
        exit(1)
    set_n_clients(int(sys.argv[1]))
