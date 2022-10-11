all: one.sample one.input two.sample two.input

one.input: ./one/bin/Debug/net6.0/one
	./one/bin/Debug/net6.0/one < input
one.sample: ./one/bin/Debug/net6.0/one
	./one/bin/Debug/net6.0/one < input.sample
./one/bin/Debug/net6.0/one: one/one.cs
	dotnet build one/one.csproj
two.input: ./two/bin/Debug/net6.0/two
	./two/bin/Debug/net6.0/two < input
two.sample: ./two/bin/Debug/net6.0/two
	./two/bin/Debug/net6.0/two < input.sample
./two/bin/Debug/net6.0/two: two/two.cs
	dotnet build two/two.csproj
