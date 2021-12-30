Feature: SessionGetProof feature

    Scenario Outline: asking for proofs with different parameters
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine for proof on cycle <cycle> for address <address> with log2_size <size>
        Then server returns proof which SHA256 sum is <hash>

        Examples:
            | cycle |        address      | size |                             hash                                 |
            |  30   |          288        |  3   | B2B4E88AC8976A6F978D4D5066AAFB85C9E1ED25150F7B686571E3924CE350F8 |
            |  30   |          288        |  4   | 61AA9A9615CD9F45CC8F982BF07687C72E043723CC54959164D4235184B1E1D3 |
