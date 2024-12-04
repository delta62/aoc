{-# OPTIONS_GHC -Wno-incomplete-patterns #-}

module Day03 (part1, part2) where

import Data.Array (Array, elems)
import Text.Regex.PCRE (MatchArray, MatchLength, MatchOffset, (=~))
import Util (parseInt)

type Matches = Array Int (MatchOffset, MatchLength)

data Op = Do | Dont | Mul Int Int deriving (Show)

-- Given a source string and an array of match locations, return the strings
-- that the regex matched. A regex without any (matching) capture groups will
-- always return a list of length 1, whereas matches including groups will
-- have one list element per matched group, plus the first element as the
-- complete match.
stringMatches :: String -> Matches -> [String]
stringMatches s xs = elems matches
  where
    extractMatch (off, len) = take len $ drop off s
    matches = fmap extractMatch xs

-- Given a match of an opcode regex, parse an op out of it
parseMatch :: [String] -> Op
parseMatch ("do()" : _) = Do
parseMatch ("don't()" : _) = Dont
parseMatch [_, x, y] = Mul (parseInt x) (parseInt y)

-- Given a set of ops, reduce them down to all of the (enabled) `mul` calls
-- added together
resolveOps :: (Foldable t) => t Op -> Int
resolveOps xs = fst $ foldl folder (0, Do) xs
  where
    folder (x, _) Do = (x, Do)
    folder (x, _) Dont = (x, Dont)
    folder (x, Do) (Mul a b) = (a * b + x, Do)
    folder (x, Dont) (Mul _ _) = (x, Dont)

-- Parse an input string into a list of operations
parse :: String -> String -> [Op]
parse re input = fmap parseOp matches
  where
    matches = input =~ re :: [MatchArray]
    parseOp = parseMatch . stringMatches input

part1 :: String -> Int
part1 input =
  let re = "mul\\((\\d+),(\\d+)\\)"
   in resolveOps $ parse re input

part2 :: String -> Int
part2 input =
  let re = "mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)"
   in resolveOps $ parse re input
