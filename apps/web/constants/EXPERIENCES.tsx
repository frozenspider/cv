import { Experience } from '../types/Experience'

// Responsibilities
// Technologies

export const EXPERIENCES: Experience[] = [
   {
      company: 'Nethermind',
      link: 'https://www.nethermind.io/',
      badges: ['Remote'],
      title: 'Sr Software Engineer (Rust)',
      logo: undefined,
      start: 'Jun 2024',
      end: undefined,
      description: (
         <>
            I'm working on several internal web3-related projects related to polynomial commitment, data availability sampling, post-quantum cryptography and zkVM execution using Rust-based tech stack.
         </>
      ),
   },

   {
      company: 'Yugabyte',
      link: 'https://www.yugabyte.com/',
      badges: ['Remote'],
      title: 'Sr Software Engineer (C/C++)',
      logo: undefined,
      start: 'Jan 2019',
      end: 'Jan 2023',
      description: (
         <>
            I was working on the YSQL layer, integrating PostgreSQL query processor with YugabyteDB engine and optimizing it out. I was responsible for both design and implementation of features such as an online upgrade framework to allow bringing PostgreSQL metadata to date on the fly, and tablegroups to allow multiple tables to share a sharded storage.
         </>
      ),
   },

   {
      company: 'Business Triangle',
      link: undefined,
      badges: [/*'Office'*/],
      title: 'Software Architect and Sr. Software Engineer (Java, Scala)',
      logo: undefined,
      start: 'Jul 2018',
      end: 'Dec 2018',
      description: (
         <>
            I was hired to redesign existing Java EE project to move from JSF/PrimeFaces to pure REST/WebSocket API
         </>
      ),
   },

   {
      company: 'Sample6',
      link: undefined,
      badges: ['Remote'],
      title: 'Lead Software Engineer (Java, Groovy)',
      logo: undefined,
      start: 'Aug 2014',
      end: 'Feb 2017',
      description: (
         <>
            Full-stack developer on Groovy/Grails technology stack.
            <br/>
            Responsibilities included support of heavy legacy codebase, gradual vertical refactoring, architecture and system design on both front-end and back-end.
         </>
      ),
   },

   {
      company: 'CompStak',
      link: 'https://compstak.com/',
      badges: ['Remote'],
      title: 'Sr. Software Engineer (Java, Groovy, Scala)',
      logo: undefined,
      start: 'Aug 2012',
      end: 'Jul 2014',
      description: (
         <>
            Backend Groovy/Grails developer, later moved to Scala/Slick/Spray technology stack. Responsibilities included designing complex extendable architectures from scratch, investigating and comparing new technologies and libraries, iterative refactoring and extensive test coverage.
         </>
      ),
   },

   {
      company: 'Itilect Inc.',
      link: undefined,
      badges: [],
      title: 'Software Engineer (Java)',
      logo: undefined,
      start: 'Nov 2010',
      end: 'May 2012',
      description: (
         <>
            I was the only software developer in a small company, so my responsibilities included everything from architecture design to final product testing.
         </>
      ),
   },
]
