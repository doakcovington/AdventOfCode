import * as fs from 'fs';

const calCount = (input: string): number => {
  const file = fs.readFileSync(input, 'utf8');
  const data = file.split('\n');
  
  let temp: number[] = [];
  let current: number = 0;
  let result: number[] = []
  
  for (let index = 0; index < data.length; index++) {
    if (data[index].length != 0) {
      let num: number = parseInt(data[index]);
      temp.push(num)
    } else {
      current = temp.reduce((sum, a) => sum + a, 0);
      result.push(current);
      temp = [];
    }
  }
  
  result.sort((low, high) => high - low);
  
  return result.slice(0, 3).reduce((sum, v) => sum + v, 0);
}

 console.log(calCount('./input.txt'))
