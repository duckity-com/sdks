"use client";

import { Button } from "@/components/ui/button";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useChallenge } from "@duckity/react";
import { SubmitEvent, useState } from "react";

export default function Home() {
  const challenge = useChallenge("idp-r4-dpQmaQYq1s4uAW");
  const [status, setStatus] = useState<"idle" | "loading">("idle");

  async function handleSubmit(e: SubmitEvent<HTMLFormElement>) {
    e.preventDefault();
    setStatus("loading");

    const solution = await challenge.wait();

    setStatus("idle");
    alert(`Submitted!\n\nThe solution is: ${solution}`);

    // This example doesn't submit anything, so we refresh the challenge to always have a valid one
    // when the user presses log in.
    challenge.refresh();
  }

  return (
    <main className="py-16 px-8">
      <form onSubmit={handleSubmit}>
        <FieldSet>
          <FieldLegend>@duckity/react Example</FieldLegend>
          <FieldDescription>
            Fill out the following login form with fake login credentials.
          </FieldDescription>
          <FieldGroup>
            <Field>
              <FieldLabel>Username or Email</FieldLabel>
              <Input />
            </Field>
            <Field>
              <FieldLabel>Password</FieldLabel>
              <Input type="password" />
            </Field>
          </FieldGroup>
          <Field>
            <Button
              type="submit"
              className="w-full"
              disabled={status == "loading"}
            >
              {status == "loading" ? "Logging In..." : "Log In"}
            </Button>
            <Button
              type="button"
              variant="outline"
              className="w-full"
              disabled={"solving" == challenge.status}
              onClick={() => {
                challenge.refresh();
              }}
            >
              Refresh Challenge
            </Button>
            {challenge.error && <FieldError>{challenge.error.toString()}</FieldError>}
          </Field>
        </FieldSet>
      </form>
    </main>
  );
}
