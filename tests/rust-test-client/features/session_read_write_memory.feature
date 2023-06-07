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

Feature: SessionReadWriteMemory feature

    Scenario Outline: read from pristine machine
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        When client asks server to read memory on cycle <cycle> and ucycle <ucycle>, starting on address <address> for length <length>
        Then server returns read bytes <bytes>

        Examples:
            | cycle | ucycle |      address      | length |              bytes               |
            |   1   |   200    | 36028797018963970 |   16   | 00000000000000000000000000000000 |
            |  30   |   400   | 36028797018963970 |   16   | 00000000000000000000000000000000 |

    Scenario Outline: read written value
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        And the write request executed for cycle <cycle> and ucycle <ucycle>, starting address <address> and data <data>
        When client asks server to read memory on cycle <cycle> and ucycle <ucycle>, starting on address <address> for length <length>
        Then server returns read bytes <bytes>

        Examples:
            | cycle | ucycle |      address       | length |    data    |             bytes                |
            |  30   |  150   |  36028797018963970 |   16   | HELLOWORLD | 48454C4C4F574F524C44000000000000 |

    Scenario Outline: read on invalid cycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 40 final ucycles
        When client asks server to read memory on cycle <cycle> and ucycle 40, starting on address <address> for length <length>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | cycle |      address      | length |
            |   5   | 36028797018963970 |   16   |

    Scenario Outline: read on invalid ucycle

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 40 final ucycles
        When client asks server to read memory on cycle 20 and ucycle <ucycle>, starting on address <address> for length <length>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | ucycle |      address      | length |
            |   5    | 36028797018963970 |   16   |

    Scenario Outline: write on invalid cycle

        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 15 final ucycles
        When client asks server to write data <data> on cycle <cycle> and ucycle 15, starting on address <address>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | cycle |        address    |    data    |
            |   5   | 36028797018963970 | HELLOWORLD |

    Scenario Outline: write on invalid ucycle

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 15 final ucycles
        When client asks server to write data <data> on cycle 20 and ucycle <ucycle>, starting on address <address>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | ucycle |        address    |    data    |
            |   5    | 36028797018963970 | HELLOWORLD |
