import { HttpAgent, Actor } from '@dfinity/agent'

export async function getAgent(authenticated = false) {
  const host = import.meta.env.VITE_HOST || 'http://127.0.0.1:4943'
  const agent = new HttpAgent({ host })
  
  if (import.meta.env.VITE_DFX_NETWORK === 'local') {
    await agent.fetchRootKey()
  }
  return agent
}

export async function getActor(canisterId: string, idlFactory: any, authenticated = false) {
  const agent = await getAgent(authenticated)
  return Actor.createActor(idlFactory, { agent, canisterId }) as any
}