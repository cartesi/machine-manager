Feature: SessionReadWriteMemory feature

    Scenario Outline: SessionReadWriteMemory read from pristine machine
        Given a machine manager server with a machine executed for <cycle> final cycles
        When client asks server to read memory on cycle <cycle>, starting on address <address> for length <length>
        Then server returns read bytes <bytes>

        Examples:
            | cycle |        address      | length |              bytes               |
            |   1   | 9223372036854775808 |   16   | 00000000000000000000000000000000 |
            |  30   | 9223372036854775808 |   16   | 00000000000000000000000000000000 |

    Scenario Outline: SessionReadWriteMemory read written value
        Given a machine manager server with a machine executed for <cycle> final cycles
        And the write request executed for cycle <cycle>, starting address <address> and data <data>
        When client asks server to read memory on cycle <cycle>, starting on address <address> for length <length>
        Then server returns read bytes <bytes>

        Examples:
            | cycle |       address       | length |    data    |             bytes                |
            |  30   | 9223372036854775808 |   16   | HELLOWORLD | 48454C4C4F574F524C44000000000000 |

    Scenario Outline: SessionReadWriteMemory read on invalid cycle

        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given a machine manager server with a machine executed for 20 final cycles
        When client asks server to read memory on cycle <cycle>, starting on address <address> for length <length>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | cycle |        address      | length |
            |   5   | 9223372036854775808 |   16   |

    Scenario Outline: SessionReadWriteMemory write on invalid cycle

        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given a machine manager server with a machine executed for 20 final cycles
        When client asks server to write data <data> on cycle <cycle>, starting on address <address>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | cycle |        address      |    data    |
            |   5   | 9223372036854775808 | HELLOWORLD |
