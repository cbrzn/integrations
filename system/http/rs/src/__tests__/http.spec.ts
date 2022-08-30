import {PolywrapClient} from "@polywrap/client-js";
import path from "path";

jest.setTimeout(500000);

describe("Http", () => {
    const wrapperPath: string = path.join(
        path.resolve(__dirname),
        "..",
        ".."
    );
    const wrapperUri = `fs/${wrapperPath}/build`;

    const client: PolywrapClient = new PolywrapClient();

    test("should get", async () => {
        const ping = await client.invoke({
            uri: wrapperUri,
            method: "get",
            args: {
                url: "https://github.dev/defiwrapper/coingecko-rs"
            }
        })
        expect(ping.error).toBeFalsy();
        expect(ping.data).toBeTruthy();
        // expect(ping.data?.gecko_says).toStrictEqual("(V3) To the Moon!");
    });
});
