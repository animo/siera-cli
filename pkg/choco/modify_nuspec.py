#!/usr/bin/env python3

"""Small utility to manipulate the agent-cli.nuspec - a file
required for the chocolatey release"""

from xml.dom import minidom
import sys

# Get version to be set for agent-cli release from env
VERSION = sys.argv[1]

if not VERSION:
    print('Failed to get version for agent-cli from system\n')
    exit(0)
else:
    print(f"Found version {VERSION} for agent-cli in environment\n")

if VERSION.startswith('v'):
    # Truncate the v prefix if present
    VERSION = VERSION[1:]

try:
    #  Get the xml from file
    with open('agent-cli.nuspec', 'r+') as nuspec:
        xml_file = minidom.parse(nuspec)

        # Find version tag
        version = xml_file.getElementsByTagName('version')[0]
        print(f"Found version tag with version: {version.childNodes[0].nodeValue}")

        # set the version tag
        version.childNodes[0].nodeValue = VERSION
        print(f"Set new version: {version.childNodes[0].nodeValue}")

        # Write the updates to file - overwrite
        nuspec.seek(0)
        nuspec. write(xml_file.toxml())
        nuspec.truncate()
except Exception as e:
    print("Failed to update .nuspec file\n")
    print(f"{e!r}")
    exit(0)
