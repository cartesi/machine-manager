Feature: SessionRun feature

    Scenario: run from pristine machine
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 0,15,30,45,60 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   0    | 27688E9BDB89F7ED6C7259720D23FA93C3C2102A614A34B2FF3D42FF8B95BE6E |
            |   15   | 6792D74EF37DA87126FD276E4AA1A0F18520FF7828C1981DE45120F2D7AA08BC |
            |   30   | A9904C4F02092E54B4EB1FE2490E149632544DF77C4F5EDDCAF2FA47FE1640FE |
            |   45   | BCFA197858A199EF924150D7F3AA1BC8B35352334550519AC1D46A22C54AAAE8 |
            |   60   | DC679DFF5089A2F9E736EF6C421C5AE1F5B6EA033640109F78230E267FC2D812 |

    Scenario: run with rollback forcing

        # If first final_cycle < machine cycle and first final_cycle > snapshot cycle,
        # machine manager server forces cartesi machine rollback

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 15,30,45 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   15   | 6792D74EF37DA87126FD276E4AA1A0F18520FF7828C1981DE45120F2D7AA08BC |
            |   30   | A9904C4F02092E54B4EB1FE2490E149632544DF77C4F5EDDCAF2FA47FE1640FE |
            |   45   | BCFA197858A199EF924150D7F3AA1BC8B35352334550519AC1D46A22C54AAAE8 |

    Scenario: run with recreation forcing

        # If first final cycle < machine cycle and first final cycle < snapshot cycle,
        # machine manager server forces cartesi machine recreation.

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 1,5,10 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   1    | AF362E5BF0CC2746B75D272FAA0ADD7E821671E2B074A057C796CC187253C29F |
            |   5    | 05D049BA6779C38AB1C563166E664495E1ACB22C864B7DE2FBFE974FF39E4110 |
            |   10   | FA85F7B995A21BE9AFDAAB9F38639F5C814102FD1F56ECC210F219F64489C452 |

    Scenario: run with no need for any special effort

        # If first final cycle > machine cycle no special effort should be needed.

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 15 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   15   | 6792D74EF37DA87126FD276E4AA1A0F18520FF7828C1981DE45120F2D7AA08BC |

    Scenario: long run
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 500000000 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            |  cycles   | hash                                                             |
            | 500000000 | 029F0C9F2129534C77E901D7333FB06438F14F7232FECC643C5791E06AC9789C |
