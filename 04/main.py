import re

def part_1():
    score_total = 0
    with open("input.txt", "r") as f:
        for line in f.readlines():
            score = 0
            parts = re.split("\s+", line)
            for winner in parts[2:12]:
                if winner in parts[13:38]:
                    score = 1 if score == 0 else score * 2
            score_total += score
    print("Part 1, points: %i" % score_total)

class GamePart2:
    def __init__(self) -> None:
    # Process file into a list of lists
        self.cards_original = []
        with open("input.txt", "r") as f:
            self.cards_original = []
            for line in f.readlines():
                parts = re.split("\s+", line)
                self.cards_original.append([parts[2:12], parts[13:38]])

    def run(self):
        cards = 0
        for card_idx in range(len(self.cards_original)):
            cards += self.process_card(card_idx)
        print("Part 2, cards: %i" % cards)

    def process_card(self, card_idx, depth=0):
        cards = 1
        winners = self.winning_numbers(self.cards_original[card_idx][0], self.cards_original[card_idx][1])
        for i in range(1, winners+1):
            cards += self.process_card(card_idx + i, depth=depth+1)
        return cards
        

    def winning_numbers(self, winners, candidates):
        n = 0
        for winner in winners:
            if winner in candidates:
                n += 1
        return n

def part_2():
    game = GamePart2()
    game.run()


if __name__ == "__main__":
    part_1()
    part_2()
