# replicatEdu Intermediate Milestones

The following is a short interactive tutorial that uses key functionality of relicatEdu to show how the instructor workflow will take place.  This does not reflect the final workflow, but will introduce serveral concepts that replicaatEdu will address in the final release.  The file https://github.com/replicatedu/replicatedu_lib/blob/master/src/main.rs was created for the intemediate milestones specifically to outline some of the workflows that the final project will be encompassing.

# Introduction

The key problem I am trying to solve in replicatEdu was having one maintainable and testible code base that could be quickly split into solution and student assignment files.  This first deliverable attempts to capture some of that workflow.  This will be an interactive walkthrough that will take about 5 to 10 minutes to complete.  

My final project will be the development an open modular educational platform focused on capturing reproducible lab, project and research environments I am now calling ReplicatEdu that allows for the development and execution of laboratory, project and research-based portions of computer science classes.  This platform will have two intended user groups, instructors/teaching assistants and students accomplishing those labs or creating their own research environments.  This portion covers how classes will be laid out and tested.

The link to my full proposal is here: https://docs.google.com/document/d/1e88dNjZeNQmzbyCx1qZkphCyfpnPW3tSV9dDnC8z8EQ/edit?usp=sharing

Please launch the workspace by clicking the below button on logging in with your github account

[![Open in Gitpod](http://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io#https://github.com/replicatedu/replicatedu_lib/)

This workspace will be preconfigured to allow you to perform all the nessecary steps.  

# Compile the Code

This first step is to compile the code which should already be in your main directory.  Please make sure you are in the project directory.  This is a rust application and can be compiled with

```
cargo build
```

After compilation (ignore all the warnings), you can run the program with the following arguments `#cargo run [class repo] [output directory]`

```
cargo run https://github.com/replicatedu/test_class.git test
```

This will perform several operations.  Annotated output is below:

```
creating output directory: 
	done
```

This creates a new directory that the original class will be cloned into and the resultant student and solution workspaces will be created

```
pulling class repository: 
	done
```

This downloads the class repository into the output directory

```
creating student and solution directories
test/test_class/assignment_1/assignment1.sh
...
test/test_class_student/README.md
	done

```
The next step creates a student and solution directory.  It starts by copying all of the files, then iterating through each of them and parsing out the skeleton and solution code.  More can be read about this process and the library I wrote to do it here: https://github.com/replicatedu/skeleton_parser.
```
Running Test Files:  test/test_class_solution/assignment_1/manifest.replicatedu
...
Running Test Files:  test/test_class_student/manifest.replicatedu
Test { description: Some("this is a test to check hello world"), test_type: Some("test_assert_err"), test_directory: Some("."), exit_code: None, target_threshold: None, cmd: Some("asd world"), expected: Some("hello world"), points: Some(10) }
Test { description: Some("this is a test to check hello world"), test_type: Some("test_assert_exit_code"), test_directory: Some("."), exit_code: Some(0), target_threshold: None, cmd: Some("echo hello world"), expected: Some("hello world"), points: Some(10) }
[src/main.rs:69] scores = [
    10,
    10
]
	done
```

This is using the custom test library that will be used to run through all of the tests in both the solution and the student directory.  This will allow any instructor to fully test student and solution issued projects.  

Please go inspect test_students and test_solution.  Test students is the files you would assign and test_solution is made to ensure automated tests still pass.

# Future Work

At this point, this is a prototype for a small portion of the overall project.  Eventually these split assignments will be packaged into reproducible environments that students will download using features of the platform still in development.  The tests can also be marked in a variety of ways and can be seperated into student and instructor tests.  The next deliverable will focus on the student portion of this framework.
