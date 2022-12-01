var lineReader = require('readline').createInterface({
	input: require('fs').createReadStream('input.txt')
});

let highest = 0;
let bucket = 0;

let top3 = [];
function add(num) {
	top3.push(num);
	top3.sort((a,b) => a - b);
	if (top3.length > 3)
		top3.splice(0, 1);
}

lineReader.on('line', function (line) {
	if (line.trim() === "") {
		add(bucket);
		bucket = 0;
	} else {
		bucket += parseInt(line.trim());
	}
});

lineReader.on('close', function () {
	add(bucket);
	console.log(top3);
	console.log(top3.reduce((a, b) => a + b, 0));
});
