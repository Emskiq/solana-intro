import * as borsh from 'borsh';
import * as math from './math';


 // THIS IS OUR ACCOUNT DATA

class CalculatorStuff {
  value = 0;
  constructor(fields: {value: number} | undefined = undefined) {
    if (fields) {
      this.value = fields.value;
    }
  }
}

const CalcStuffSchema = new Map([
  [CalculatorStuff, {kind: 'struct', fields: [['value', 'u32']]}],
]);

const CALC_STUFF_SIZE = borsh.serialize(
  CalcStuffSchema,
  new CalculatorStuff(),
).length;



async function main() {
  await math.example('calculator', CALC_STUFF_SIZE);
}


main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );
