import { useJournalClient } from "../utils/useJournalClient";

export default defineEventHandler(async (event) => {
  const query = getQuery(event);
  const journalClient = useJournalClient(event);

  if (query.id) {
    return journalClient.findById(query.id?.toString());
  }
});
