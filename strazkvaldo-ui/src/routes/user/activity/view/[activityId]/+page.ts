import type { pageload } from './$types';

export const load: pageload = async ({ params, parent }) => {
    let { oneTimeActivities, repeatedActivities } = await parent() ?? {};
    return Array.prototype.concat(oneTimeActivities ?? [], repeatedActivities ?? []).find((a) => a.id == params.activityId);
};