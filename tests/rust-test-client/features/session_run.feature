Feature: SessionRun feature

    Scenario: run from pristine machine
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 0,15,30,45,60 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   0    | E6B64F49947D3AC83F7B54E57E47081657813AB679A433B7F5D3E1D136F66072 |
            |   15   | 4BD8412455EBB67328A5FF8B9777DD1AF10C71EF8A8C3F6E92F06292090A4A4D |
            |   30   | BB9CFEFF538D6C2D956D88D2E243C4496B819E59B63C13EF5016506C586006AB |
            |   45   | 86775B6D8B1DB70C6AEE81A448F29879F20F82F0C5907D6C43964FC49AF23C30 |
            |   60   | 2A263E0E58691C40E6F345BBCFD5BA307F7DD0F4CED6D84CE1DEEB3765A2102B |

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
            |   15   | 4BD8412455EBB67328A5FF8B9777DD1AF10C71EF8A8C3F6E92F06292090A4A4D |
            |   30   | BB9CFEFF538D6C2D956D88D2E243C4496B819E59B63C13EF5016506C586006AB |
            |   45   | 86775B6D8B1DB70C6AEE81A448F29879F20F82F0C5907D6C43964FC49AF23C30 |

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
            |   1    | 56B0404F692CB6EFDA1E8088164D70FB35A29751BB7AB5EE493B02059A06B176 |
            |   5    | 3ADA98431056653629CC252A60CD59EAFEF39FD8A1E6CB8C004B88890B2BF662 |
            |   10   | 92C1A0FF2EDE625FFF78AB87C8537EB216D9B8044C5BAD5F1D3BC5ADEA0F72AA |

    Scenario: run with no need for any special effort

        # If first final cycle > machine cycle no special effort should be needed.

        Given machine manager server is up
        And a pristine machine manager server session
        And the machine executed with cycles 0,30,60
        And the cycles array 15 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   15   | 4BD8412455EBB67328A5FF8B9777DD1AF10C71EF8A8C3F6E92F06292090A4A4D |

    Scenario: long run
        Given machine manager server is up
        And a pristine machine manager server session
        And the cycles array 500000000 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            |  cycles   | hash                                                             |
            | 500000000 | F1035CAB45480B43DFA5F7B2FDE2ECAD1B44EF08D1EF7D02A598A3010AE4ACD7 |
