# Copyright 2023 Cartesi Pte. Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use
# this file except in compliance with the License. You may obtain a copy of the
# License at http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed
# under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
# CONDITIONS OF ANY KIND, either express or implied. See the License for the
# specific language governing permissions and limitations under the License.

Feature: SessionRun feature

    Scenario: run from pristine machine
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 0,15,30,45,60 to run the machine
        When client asks server to run session
        Then server returns correct machine hashes

    Scenario: run with rollback forcing

        # If first final_cycle < machine cycle and first final_cycle > snapshot cycle,
        # machine manager server forces cartesi machine rollback

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 15,30,45 to run the machine
        When client asks server to run session
        Then server returns correct machine hashes

    Scenario: run with recreation forcing

        # If first final cycle < machine cycle and first final cycle < snapshot cycle,
        # machine manager server forces cartesi machine recreation.

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 1,5,10 to run the machine
        When client asks server to run session
        Then server returns correct machine hashes

    Scenario: run with no need for any special effort

        # If first final cycle > machine cycle no special effort should be needed.

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 15 to run the machine
        When client asks server to run session
        Then server returns correct machine hashes

    Scenario: long run
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 500000000 to run the machine
        When client asks server to run session
        Then server returns correct machine hashes
