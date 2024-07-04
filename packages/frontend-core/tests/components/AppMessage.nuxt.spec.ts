import { it, expect } from "vitest";
import { mountSuspended } from "@nuxt/test-utils/runtime";
import { AppMessage } from "#components";

it("can mount some component", async () => {
  const component = await mountSuspended(AppMessage, { props: { modelValue: "Test 12345" } });
  expect(component.text()).toMatchInlineSnapshot('"New From App Message:Test 12345"');
});
