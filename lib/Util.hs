module Util where

parseInt :: String -> Int
parseInt = read

count :: (a -> Bool) -> [a] -> Int
count f xs = length $ filter f xs

mapAdjacent :: (a -> a -> b) -> [a] -> [b]
mapAdjacent _ [] = []
mapAdjacent _ [_] = []
mapAdjacent f (x:y:xs) = f x y : (mapAdjacent f (y:xs))
