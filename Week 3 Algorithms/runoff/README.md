# Problem to Solve

You already know about plurality elections, which follow a very simple algorithm for determining the winner of an election: every voter gets one vote, and the candidate with the most votes wins.

But the plurality vote does have some disadvantages. What happens, for instance, in an election with three candidates, and the ballots below are cast?

Five ballots, tie betweeen Alice and Bob

A plurality vote would here declare a tie between Alice and Bob, since each has two votes. But is that the right outcome?

There’s another kind of voting system known as a ranked-choice voting system. In a ranked-choice system, voters can vote for more than one candidate. Instead of just voting for their top choice, they can rank the candidates in order of preference. The resulting ballots might therefore look like the below.

Five ballots, with ranked preferences

Here, each voter, in addition to specifying their first preference candidate, has also indicated their second and third choices. And now, what was previously a tied election could now have a winner. The race was originally tied between Alice and Bob, so Charlie was out of the running. But the voter who chose Charlie preferred Alice over Bob, so Alice could here be declared the winner.

Ranked choice voting can also solve yet another potential drawback of plurality voting. Take a look at the following ballots.

Nine ballots, with ranked preferences

Who should win this election? In a plurality vote where each voter chooses their first preference only, Charlie wins this election with four votes compared to only three for Bob and two for Alice. But a majority of the voters (5 out of the 9) would be happier with either Alice or Bob instead of Charlie. By considering ranked preferences, a voting system may be able to choose a winner that better reflects the preferences of the voters.

One such ranked choice voting system is the instant runoff system. In an instant runoff election, voters can rank as many candidates as they wish. If any candidate has a majority (more than 50%) of the first preference votes, that candidate is declared the winner of the election.

If no candidate has more than 50% of the vote, then an “instant runoff” occurrs. The candidate who received the fewest number of votes is eliminated from the election, and anyone who originally chose that candidate as their first preference now has their second preference considered. Why do it this way? Effectively, this simulates what would have happened if the least popular candidate had not been in the election to begin with.

The process repeats: if no candidate has a majority of the votes, the last place candidate is eliminated, and anyone who voted for them will instead vote for their next preference (who hasn’t themselves already been eliminated). Once a candidate has a majority, that candidate is declared the winner.

Sounds a bit more complicated than a plurality vote, doesn’t it? But it arguably has the benefit of being an election system where the winner of the election more accurately represents the preferences of the voters. In a file called runoff.c in a folder called runoff, create a program to simulate a runoff election.

# Distribution Code
## Download the distribution code

Understand the code in runoff.c

Hints

Click the below toggles to read some advice!
Complete the vote function

Complete the vote function.

- The function takes three arguments: voter, rank, and name.
- If name is a match for the name of a valid candidate, then you should update the global preferences array to indicate that the voter voter has that candidate as their rank preference. Keep in mind 0 is the first preference, 1 is the second preference, etc. You may assume that no two candidates will have the same name.
- If the preference is successfully recorded, the function should return true. The function should return false otherwise. Consider, for instance, when name is not the name of one of the candidates.

As you write your code, consider these hints:

- Recall that candidate_count stores the number of candidates in the election.
- Recall that you can use strcmp to compare two strings.
- Recall that preferences[i][j] stores the index of the candidate who is the jth ranked preference for the ith voter.

## Complete the tabulate function

- The function should update the number of votes each candidate has at this stage in the runoff.
- Recall that at each stage in the runoff, every voter effectively votes for their top-preferred candidate who has not already been eliminated.

As you write your code, consider these hints:

- Recall that voter_count stores the number of voters in the election and that, for each voter in our election, we want to count one ballot.
- Recall that for a voter i, their top choice candidate is represented by preferences[i][0], their second choice candidate by preferences[i][1], etc.
- Recall that the candidate struct has a field called eliminated, which will be true if the candidate has been eliminated from the election.
- Recall that the candidate struct has a field called votes, which you’ll likely want to update for each voter’s preferred candidate.
- Recall that once you’ve cast a vote for a voter’s first non-eliminated candidate, you’ll want to stop there, not continue down their ballot. You can break out of a loop early using break inside of a conditional.

## Complete the print_winner function

- If any candidate has more than half of the vote, their name should be printed and the function should return true.
- If nobody has won the election yet, the function should return false.

As you write your code, consider this hint:

- Recall that voter_count stores the number of voters in the election. Given that, how would you express the number of votes needed to win the election?

## Complete the find_min function

- The function should return the minimum vote total for any candidate who is still in the election.

As you write your code, consider this hint:

- You’ll likely want to loop through the candidates to find the one who is both still in the election and has the fewest number of votes. What information should you keep track of as you loop through the candidates?

## Complete the is_tie function

- The function takes an argument min, which will be the minimum number of votes that anyone in the election currently has.
- The function should return true if every candidate remaining in the election has the same number of votes, and should return false otherwise.

As you write your code, consider this hint:

- Recall that a tie happens if every candidate still in the election has the same number of votes. Note, too, that the is_tie function takes an argument min, which is the smallest number of votes any candidate currently has. How might you use min to determine if the election is a tie (or, conversely, not a tie)?

## Complete the eliminate function

- The function takes an argument min, which will be the minimum number of votes that anyone in the election currently has.
- The function should eliminate the candidate (or candidates) who have min number of votes.

# How to Test

Be sure to test your code to make sure it handles…

- An election with any number of candidate (up to the MAX of 9)
- Voting for a candidate by name
- Invalid votes for candidates who are not on the ballot
- Printing the winner of the election if there is only one
- Not eliminating anyone in the case of a tie between all remaining candidates
