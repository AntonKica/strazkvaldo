import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		settings: {
			criticality: "low",
			notifyDaysBefore: 2,
        }
	};
};