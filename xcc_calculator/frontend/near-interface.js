import { utils } from "near-api-js";
export class Contract {
  wallet;

  constructor({ wallet }) {
    this.wallet = wallet;
  }

  async queryLastOperationResult() {
    try {
      return await this.wallet.callMethod({
        method: "query_last_operation_result",
      });
    } catch (error) {
      console.log(error);
      return "ERROR";
    }
  }

  async performOperation(op_1 = 0, op_2 = 0, operator = "NONE") {
    try {
      const deposit = utils.format.parseNearAmount(`${1}`);

      return await this.wallet.callMethod({
        method: "perform_operation",
        args: { op_1, op_2, operator },
        deposit,
      });
    } catch (error) {
      console.log(error);
      return "ERROR";
    }
  }
}
