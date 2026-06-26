import type { TopRightAdState } from '../types/topRightAd';

export async function getTopRightAdState(): Promise<TopRightAdState> {
  return { ad: null, ads: [] };
}
