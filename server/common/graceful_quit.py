class GracefulQuitFlag(object):
	def __init__(self):
		self._enabled = False

	def isEnabled(self):
		return self._enabled

	def enable(self):
		self._enabled = True