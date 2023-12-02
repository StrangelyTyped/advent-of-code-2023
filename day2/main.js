const fs = require("fs").promises

const read = async(file) => (await fs.readFile(file, "utf8")).trim()

const setup = new Map([
    ["red", 12],
    ["green", 13],
    ["blue", 14],
])

const part1 = (input) => {
    const lines = input.trim().split("\n");
    let sum = 0;
    lineLoop: for(let line of lines){
        const p1 = line.split(":")
        const game = parseInt(p1[0].split(" ")[1].trim())
        const phases = p1[1].split(";")
        for(let phase of phases){
            const parts = phase.split(", ")
            for(let part of parts){
                const p2 = part.trim().split(" ");
                const count = parseInt(p2[0].trim())
                const typ = p2[1].trim()
                if(setup.get(typ) < count){
                    continue lineLoop
                }
            }
        }
        sum += game
    }
    return sum;
}

const part2 = (input) => {
    const lines = input.trim().split("\n");
    let sum = 0;
    for(let line of lines){
        const mins = new Map([["red", 0], ["green", 0], ["blue", 0]])
        const p1 = line.split(":")
        const game = parseInt(p1[0].split(" ")[1].trim())
        const phases = p1[1].split(";")
        for(let phase of phases){
            const parts = phase.split(", ")
            for(let part of parts){
                const p2 = part.trim().split(" ");
                const count = parseInt(p2[0].trim())
                const typ = p2[1].trim()
                mins.set(typ, Math.max(mins.get(typ), count))
            }
        }
        let gameScore = 1
        for(let ball of mins){
            gameScore *= ball[1];
        }
        sum += gameScore
    }
    return sum;
}

const main = async () => {
	const test1 = await read("inputs/test1.txt")
	const input = await read("inputs/input.txt")
	console.log("Test 1", part1(test1))
	console.log("Part 1", part1(input))
	console.log("Test 2", part2(test1))
	console.log("Part 2", part2(input))
}
main()
