input = IO.read(:stdio, :all)
f = Regex.replace(~r/up /, 
        Regex.replace(~r/down /, 
            Regex.replace(~r/forward /, input, "horizontal = horizontal + "), 
            "depth = depth + "),
        "depth = depth - ") <> "horizontal * depth"
{result, _binding} = Code.eval_string(f, [horizontal: 0, depth: 0], __ENV__)
IO.puts(result)
