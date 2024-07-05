import { useJournalClient } from "../utils/useJournalClient";

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  const journalClient = useJournalClient(event);

  if (body) {
    return journalClient.handleCommand(body);
  }
});
