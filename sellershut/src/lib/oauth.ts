export const toOauthCookieName = (name: string): { provider: string; cookieName: string } => {
  const provider = name.toLowerCase().trim();
  const cookieName = `oauth_state_${provider}`;
  return { provider, cookieName };
};
