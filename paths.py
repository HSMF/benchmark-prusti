from os import environ

PRUSTI_DEV_SOURCE = environ.get("PRUSTI_DEV_SOURCE", "../prusti-dev")
VIPERSERVER = environ.get(
    "VIPERSERVER", "../prusti-dev/viper_tools/backends/viperserver.jar"
)
SILICON_SOURCE = environ.get("SILICON_SOURCE", "/home/hyde/eth/10/pv/silicon")
