i = 1000

loop i = do
    if i > 0 then do
        putStrLn (show i ++ " - 7 = " ++ show (i-7))
        loop (i-7)
    else return ()

main = loop i