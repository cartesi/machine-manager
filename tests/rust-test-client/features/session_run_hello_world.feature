Feature: SessionRun hello world feature

    Background:
        Given a pristine machine manager hello world server session

        # Machine should halt and return end hash before reaching final cycle
    Scenario: SessionRun long run with halt
        Given the cycles array 1000000000000 to run the machine
        When client asks server to run hello world session
        Then server returns machine hashes of hello world machine:
            |  cycles       | hash                                                                  | end_cycle |
            | 1000000000000 | B0DABC4C0EBD6FF6E1551EAAB2724C7AB0481C936FEF6A89677D7BFCDB1022F8      | 68665112 |