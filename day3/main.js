const fs = require("fs").promises

const read = async(file) => (await fs.readFile(file, "utf8")).trim()

const isDigit = (chr) => chr >= '0' && chr <= '9';
const isSymbol = (chr) => chr != '.' && (chr < '0' || chr > '9');
const isGear = (chr) => chr == '*';

const compute = (input, pred) => {
    const lines = input.trim().split("\n");
    for(let y = 0; y < lines.length; y++){
        numLoop: for(let x = 0; x < lines[y].length; x++){
            if(isDigit(lines[y][x])){
                let numStr = lines[y][x]
                let xStart = x;
                x++
                while(x < lines[y].length && isDigit(lines[y][x])){
                    numStr += lines[y][x]
                    x++
                }
                for(let y2 = Math.max(0, y-1); y2 < Math.min(lines.length-1, y+2); y2++){
                    for(let x2 = Math.max(0, xStart - 1); x2 < Math.min(lines[y2].length-1, x+1); x2++){
                        if(pred(lines[y2][x2], numStr, y2, x2)){
                            continue numLoop
                        }
                    }
                }
            }
        }
    }
}

const part1 = (input) => {
    let sum = 0;

    compute(input, (chr, numStr) => {
        if(isSymbol(chr)){
            sum += parseInt(numStr)
            return true
        }
        return false
    })

    return sum;
}

const part2 = (input) => {
    let sum = 0;
    let gearMap = new Map();

    compute(input, (chr, numStr, y2, x2) => {
        if(isGear(chr)){
            let k = y2 + "/" + x2;
            if(gearMap.has(k)){
                sum += parseInt(numStr) * parseInt(gearMap.get(k));
            }else{
                gearMap.set(k, numStr)
            }
            return true
        }
        return false
    })

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
