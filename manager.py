"""
Copyright 2021 Cartesi Pte. Ltd.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
"""

import argparse
from manager_server import LISTENING_ADDRESS, LISTENING_PORT, CHECKIN_PORT, serve


if __name__ == '__main__':
    #Adding argument parser
    description = "Instantiates a machine manager server, responsible for managing and interacting with multiple cartesi machine instances"

    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        '--address', '-a',
        dest='address',
        default=LISTENING_ADDRESS,
        help='Address to listen (default: {})'.format(LISTENING_ADDRESS)
    )
    parser.add_argument(
        '--port', '-p',
        dest='port',
        default=LISTENING_PORT,
        help='Port to listen (default: {})'.format(LISTENING_PORT)
    )
    parser.add_argument(
        '--checkin', '-c',
        dest='checkin',
        default=CHECKIN_PORT,
        help='Checkin service port (default: {})'.format(CHECKIN_PORT)
    )
    parser.add_argument(
        '--defective', '-d',
        dest='defective',
        action='store_true',
        help='Makes server behave improperly, injecting errors silently in the issued commands\n\n' + '-'*23 + 'WARNING!' + '-'*23 + 'FOR TESTING PURPOSES ONLY!!!\n' + 54*'-'
    )
    serve(parser.parse_args())
