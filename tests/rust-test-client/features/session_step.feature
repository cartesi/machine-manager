Feature: SessionStep feature

    Scenario Outline: SessionStep steps with different cycles

        # - Step with initial cycle = 0 should happen on a new machine.
        # - Step with initial cycle < machine cycle and initial cycle > snapshot cycle should force machine rollback.
        # - Step with initial cycle < machine cycle and initial cycle < snapshot cycle should force machine recreation.
        # - Step with initial cycle > machine cycle should not require any special effort.
        # - Step with initial cycle = machine cycle, should not even require making a dummy run to get into machine
        #   cycle = initial cycle.

        Given a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine to step on initial cycle <cycle>
        Then server returns access log which SHA256 sum is <hash>

        Examples:
            | cycle |                             hash                                 |
            |   1   | 7D5C0B9D5E2A08F85501D3C4293DCE4AEFE3F4FD95A1BB84591968D23F3CC31F |
            |   21  | 9FD14CC81C631E48298512417CE02895DA6C4A49461619E4CE2193802935DF31 |
            |   35  | 740667F16511E82F2A42951275C0C891ED4AC90F942E0617CD9B8101FC39830B |
            |   30  | EC8D1F44586393FC21B2F2B3683634714BE5DB1F22EEFEE5D0988BCBB33D0EE9 |

    Scenario: SessionStep step on invalid cycle

        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given a machine manager server with a machine executed for 20 final cycles
        When the machine manager server asks machine to step on initial cycle 5
        Then machine manager server returns an InvalidArgument error
