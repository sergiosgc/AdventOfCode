input = IO.read(:stdio, :all)
f = Regex.replace(~r/up /, 
        Regex.replace(~r/down /, 
            Regex.replace(~r/forward (\d+)/, input, "horizontal = horizontal + \\1\ndepth = depth + aim * \\1"), 
            "aim = aim + "),
        "aim = aim - ") <> "horizontal * depth"
{result, _binding} = Code.eval_string(f, [horizontal: 0, depth: 0, aim: 0], __ENV__)
IO.puts(result)

