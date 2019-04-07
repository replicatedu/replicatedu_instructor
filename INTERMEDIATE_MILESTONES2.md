
The last time i demonstraited the downloading and creation of a class from a instructuors project.

this time around i will demonstrate the automation where the 
	class is created, 
	added to the instructors github for downloading by students
	set up with proper crypto for security
		for grading
		and security of class keys 
	class registration and github assignment creation for the student 

so I have several steps to automate the creation and management of a class

the first thing I would like to show is the instructor classes being created again like last time

this is an example of the instructor running the tool to automate the creation of the repositories that 
will power the class

like last time we download and build the solution and student versions of the class
additionally a file is added that contains encrypted metadata that will allow for the course to be setup

this encrypted metadata contains the instructors public key and a deploy key that will be automatically added to the student repository so the instructor can read and post grades to each students repository.  All key creation and managment is automated as part of the platform

now the student repository and the solution repository have been created in the instructors github.  the student repository is the source for all the students skeleton code and assignment information


switching gears to student registration I will now demonstrate a student registering for a class
the student will need to be provided the instructor's public key to decrypt nessecary metadata for the course.  This can be sent via email or any other out of band communications.  this will prevent people not registered from connecting to the class infrastructure.  

the student inputs the public key and the class repository address.  The student will also have to provide their github login credentials.  

now the student downloads and creates a copy of the class repository that is private in their github.  This has all been autoated by the platform.  This repository is where the instructor will pull assignment submissions for grading.  the previously mentioned deploy key will allow read and write access for the instructor

the student repo is now created and tied upstream to the instructors for assignment submission.

To power the registration and request for grades a common class database is used.  To ensure this did not require an always on server, I created a simple database within git.  there is a small simple service running within a docker container that allows from this.  you can see the request here in git

in the future the instructor can pull this down and process all the back requests in an automated fasion.  

for the registration requests the instructor will add them to the grading roster and note the public keys for each student and the grading repository address.

For grading requests (either final submission or test checks), they will get a request with metadata telling what assignment and the projet will be run via the test framework in a docker container...scores will be posted to the student repo in an added folder with a timestamp. 

If the instructor is comfortable with the thought, the potential exists for grading in a distributed system using docker swarm but this is not ideal since other students could potentially reverse engineer the docker instance.  grading can also be ran using the student tests on the students local machine, but the instructor may want to hold some tests back to ensure the student is not gaming the test framework.  

still to be added is a nice command line interface gluing all this functionality, one for the student and the instructor.  The next part I will be undertaking is the underlying container management and student development environment.  Additionally, it will all be packaged in a class independent image.  This image can be deployed to students as a virtual machine or built to deploy in any popular cloud infrastructure.   