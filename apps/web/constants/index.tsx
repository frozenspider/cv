import { Resume } from '../types'
import {
   faLinkedin,
   faGithub,
   faTelegram,
   faNpm,
   faTwitter,
} from '@fortawesome/free-brands-svg-icons'
import { SKILLS } from './SKILLS'
import { LOCATIONS } from './LOCATIONS'
import { LANGUAGES } from './LANGUAGES'
import { EXPERIENCES } from './EXPERIENCES'
import { EDUCATIONS } from './EDUCATIONS'
import { PROJECTS } from './PROJECTS'
import { CONTRIBUTIONS } from './CONTRIBUTIONS'
import { TECHNOLOGIES } from './TECHNOLOGIES'

const NAME = "Alexander Abdugafarov"
const ROLE = "Software Engineer (Rust)"

export const WEBSITE = {
   image: `${process.env.NEXT_PUBLIC_URL}/me.jpg`,
   color: '#000',
   name: `${NAME} - ${ROLE}`,
   title: `${NAME} - ${ROLE}`,
   description: `${NAME} - ${ROLE}`,
   about: `${NAME} - ${ROLE}`,
   email: 'fswork90@gmail.com',

   linkedin: 'https://www.linkedin.com/in/frozenspider',
   github: 'https://github.com/frozenspider',

   keywords: [
   ].join(', '),
   phone: null,
}

export const RESUME: Resume = {
   name: NAME,
   nameLink: WEBSITE.github,

   locations: LOCATIONS,
   languages: LANGUAGES,

   avatar: '/me.jpg',
   avatarLink: WEBSITE.github,

   summary: ROLE,
   summaryLink: WEBSITE.github,

   about: (
      <div className="w-full flex flex-col gap-3">
         <div>
         <p>
         I'm an expert Software Engineer working professionally since 2010.
         </p>
         <br />
         <p>
         I have a deep understanding of fundamental algorithms, data structures, design patterns, principles of OOP and FP. Additionally, my expertise extends to database internals, query processing, concurrent systems and distributed consensus mechanisms. I am proficient at writing high-quality, testable code.
         </p>
         <br />
         <p>
         I'm a fast learner, I don't fear complex problems, and programming is my true passion. Leave me with a challenging task and a few people to discuss it with, and I'm a happy guy!
         </p>
         </div>


         <div>
            <b>Skills:</b> Rust • Scala • Distributed Systems • Cryptography • Web3 • MVCC • Math • Optimization • Algorithms • Data Structures • REST
         </div>
      </div>
   ),
   aboutLink: WEBSITE.linkedin,

   helpLink: WEBSITE.linkedin,

   website: WEBSITE.github,

   contact: {
      website: WEBSITE.url || '/',
      email: WEBSITE.email,
      linkedin: WEBSITE.linkedin,
      github: WEBSITE.github,
   },

   technologies: TECHNOLOGIES,

   experiences: EXPERIENCES,

   educations: EDUCATIONS,

   skills: SKILLS,

   projects: PROJECTS,

   contributions: CONTRIBUTIONS,

   characteristics: [],
}
