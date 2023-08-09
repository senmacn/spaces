import { customAlphabet, urlAlphabet } from 'nanoid';

export function getRandomId() {
  const nanoid = customAlphabet(urlAlphabet, 10);
  return nanoid();
}
