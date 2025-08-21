import { HttpAgent, Actor } from '@dfinity/agent'
import { AuthClient } from '@dfinity/auth-client'

export async function getIdentity() {
  const auth = await AuthClient.create()
  if (!(await auth.isAuthenticated())) {
    await auth.login({ identityProvider: 'https://identity.ic0.app' })
  }
  return auth.getIdentity()
}

export async function getAgent(authenticated = false) {
  const identity = authenticated ? await getIdentity() : undefined
  const agent = new HttpAgent({ identity })
  if (location.hostname === '127.0.0.1' || location.hostname === 'localhost') {
    await agent.fetchRootKey()
  }
  return agent
}

export async function getActor(canisterId: string, idlFactory: any, authenticated = false) {
  const agent = await getAgent(authenticated)
  return Actor.createActor(idlFactory, { agent, canisterId }) as any
}
