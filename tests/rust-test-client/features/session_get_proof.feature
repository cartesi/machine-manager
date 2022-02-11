Feature: SessionGetProof feature

    Scenario Outline: asking for proofs with different parameters
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine for proof on cycle <cycle> for address <address> with log2_size <size>
        Then server returns correct proof

        Examples:
            | cycle |        address      | size |
            |  30   |          288        |  3   |
            |  30   |          288        |  4   |
