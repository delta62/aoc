module Day03 (part1, part2) where

import Data.Array (Array, elems)
import Text.Regex.PCRE ((=~), MatchArray, MatchOffset, MatchLength)
import Util (parseInt)

type Matches = Array Int (MatchOffset, MatchLength)

stringMatches :: String -> Matches -> [String]
stringMatches s xs = elems $ fmap extractMatch xs
  where
    extractMatch (off, len) = take len $ drop off s

calcProduct :: String -> Matches -> Int
calcProduct s xs = product ints
  where
    ints = fmap parseInt matches
    matches = tail $ stringMatches s xs

part1 :: String -> Int
part1 input = sum pairs
  where
    matches :: [MatchArray]
    matches = input =~ "mul\\((\\d+),(\\d+)\\)"
    pairs = fmap (calcProduct input) matches

part2 :: String -> Int
part2 = undefined
