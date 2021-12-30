Feature: SessionStep feature

    Scenario Outline: steps with different cycles

        # - Step with initial cycle = 0 should happen on a new machine.
        # - Step with initial cycle < machine cycle and initial cycle > snapshot cycle should force machine rollback.
        # - Step with initial cycle < machine cycle and initial cycle < snapshot cycle should force machine recreation.
        # - Step with initial cycle > machine cycle should not require any special effort.
        # - Step with initial cycle = machine cycle, should not even require making a dummy run to get into machine
        #   cycle = initial cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine to step on initial cycle <cycle>
        Then server returns correct access log

        Examples:
            | cycle |
            |   1   |
            |   21  |
            |   35  |
            |   30  |

    Scenario Outline: step on invalid cycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine to step on initial cycle <cycle>
        #Then machine manager server returns an Internal error
        Then server returns correct access log

        Examples:
            | cycle |
            |   20  |
