module Day01 (part1, part2) where

import Data.List (sort)
import qualified Data.Set as S
import qualified Data.Map as M

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

parse' :: String -> (S.Set Int, M.Map Int Int)
parse' s = (set, hash)
  where
    bits = unzip $ map parseLine (lines s)
    set = S.fromList (fst bits)
    hash = counts (snd bits)

minDiffs :: ([Int], [Int]) -> Int
minDiffs (xs, ys) = sum $ map (\(x, y) -> abs (x - y)) $ zip as bs
  where
    as = sort xs
    bs = sort ys

scores :: M.Map Int Int -> Int
scores m = M.foldrWithKey folder 0 m
  where
    folder k v acc = acc + k * v

part1 :: String -> Int
part1 input = minDiffs $ parse input

part2 :: String -> Int
part2 input = scores parsed'
  where
    parsed = parse' input
    keys = fst parsed
    parsed' = M.restrictKeys (snd parsed) keys
