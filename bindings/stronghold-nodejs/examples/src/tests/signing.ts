import {signing} from "../../../../wasm/examples-account/src/signing";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("SIgning", async () => {
        await signing(stronghold);
    });
})
