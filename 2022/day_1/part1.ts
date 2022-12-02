import * as fs from 'fs';

const calCount = (input: string): number => {
  const file = fs.readFileSync(input, 'utf8');
  const data = file.split('\n');
  
  let temp: number[] = [];
  let current: number = 0;
  let top: number = 0;
  
  for (let index = 0; index < data.length; index++) {
    if (data[index].length != 0) {
      let num: number = parseInt(data[index]);
      temp.push(num)
    } else {
      current = temp.reduce((sum, a) => sum + a, 0);
      if (current >= top) {
        top = current;
      }
      temp = [];
    }
  }
  return top;
}

 console.log(calCount('./input.txt'))
