import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		users: [
			{
				code: "USR-0001",
				username: "jaknohrasko",
				firstName: "Jakno",
				lastName: "Hrasko",
				role: "user"
			},
			{
				code: "USR-0000",
				username: "admin",
				firstName: "Admin",
				lastName: "Admin",
				role: "admin"
			}
		]
	};
};