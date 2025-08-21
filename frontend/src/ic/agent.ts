import { HttpAgent, Actor } from '@dfinity/agent'

export async function getAgent() {
  const agent = new HttpAgent({})
  // For local dev, fetch root key
  if (location.hostname === '127.0.0.1' || location.hostname === 'localhost') {
    await agent.fetchRootKey()
  }
  return agent
}

export async function getActor(canisterId: string, idlFactory: any) {
  const agent = await getAgent()
  return Actor.createActor(idlFactory, { agent, canisterId }) as any
}
