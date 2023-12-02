const fs = require("fs").promises

const part1 = (line) => {
	return line.split("")
		.filter(c => parseInt(c) == c)
		.map(c => parseInt(c));
}

const words = ["0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
const regexp = new RegExp("^(" + words.join("|") + ")")

const part2 = (line) => {
	const out = []

	for(let i = 0; i < line.length; i++){
		if(parseInt(line[i]) == line[i]){
			out.push(parseInt(line[i]))
		}else{
			const match = line.substr(i).match(regexp)
			if(match){
				out.push(words.indexOf(match[0]))
			}
		}
	}

	return out
}

const compute = (input, extractor) => {
	return input.split("\n").map(i => {
		let z = extractor(i)
		return (z[0] * 10) + z[z.length - 1];
	})
	.reduce((a, b) => a + b, 0)

}

const read = async(file) => (await fs.readFile(file, "utf8")).trim()

const main = async () => {
	const test1 = await read("inputs/test1.txt")
	const test2 = await read("inputs/test2.txt")
	const input = await read("inputs/input.txt")
	console.log("Test 1", compute(test1, part1))
	console.log("Part 1", compute(input, part1))
	console.log("Test 2", compute(test2, part2))
	console.log("Part 2", compute(input, part2))
}
main()
