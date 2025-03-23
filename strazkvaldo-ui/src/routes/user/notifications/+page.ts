import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		upcomingActivities: [{
			id: 3,
            name: "pranie oblečenia",
			type: "washing",
			date: "2025-04-01"
        }]
	};
};