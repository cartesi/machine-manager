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

Feature: SessionStep feature

    Scenario Outline: steps with different cycles

        # - Step with initial cycle = 0 should happen on a new machine.
        # - Step with initial cycle < machine cycle and initial cycle > snapshot cycle should force machine rollback.
        # - Step with initial cycle < machine cycle and initial cycle < snapshot cycle should force machine recreation.
        # - Step with initial cycle > machine cycle should not require any special effort.
        # - Step with initial cycle = machine cycle, should not even require making a dummy run to get into machine
        #   cycle = initial cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        When the machine manager server asks machine to step on initial cycle <cycle> and ucycle <ucycle>
        Then server returns correct access log

        Examples:
            | cycle | ucycle |
            |   1   |   0    |
            |   21  |   0    |
            |   35  |   0    |
            |   30  |   0    |

    Scenario Outline: step on invalid cycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        When the machine manager server asks machine to step on initial cycle <cycle> and ucycle <ucycle>
        #Then machine manager server returns an Internal error
        Then server returns correct access log

        Examples:
            | cycle | ucycle |
            |   20  |   0    |  
