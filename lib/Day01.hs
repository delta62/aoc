{-# OPTIONS_GHC -Wno-incomplete-patterns #-}

module Day01 (parse, parse', part1, part2) where

import Data.List (sort)
import qualified Data.Map as M

type Lookup = M.Map Int Int

parseStr :: String -> Int
parseStr = read

tuple :: [a] -> (a, a)
tuple [x, y] = (x, y)

parseLine :: String -> (Int, Int)
parseLine s = tuple $ map parseStr $ words s

parse :: String -> ([Int], [Int])
parse s = unzip $ map parseLine (lines s)

counts :: (Num a, Ord a, Enum a) => [a] -> M.Map a a
counts xs = foldr (\x acc -> M.insertWith inc x 1 acc) M.empty xs
  where
    inc _ x = succ x

parse' :: String -> ([Int], Lookup)
parse' s = (set, hash)
  where
    bits = unzip $ map parseLine (lines s)
    set = fst bits
    hash = counts (snd bits)

diffs :: ([Int], [Int]) -> Int
diffs (xs, ys) = sum $ map tupleDiff $ zip as bs
  where
    as = sort xs
    bs = sort ys
    tupleDiff (x, y) = abs $ x - y

scores :: [Int] -> Lookup -> Int
scores ks m = sum $ map score ks
  where
    score k = k * (M.findWithDefault 0 k m)

part1 :: String -> Int
part1 input = diffs $ parse input

part2 :: String -> Int
part2 input =
    let (keys, entries) = parse' input
     in scores keys entries
