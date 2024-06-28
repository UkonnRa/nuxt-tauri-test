export default defineEventHandler(async (event) => {
  const { apiSecret } = useRuntimeConfig(event);
  const { id } = getQuery(event);

  const result = await fetch(`https://dummyjson.com/quotes/${id}`, {
    headers: {
      Authentation: `Bearer: ${apiSecret}`,
    },
  });

  return await result.json();
});
