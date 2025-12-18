//
//  AppDelegate.m
//  coreaudio-apple-example
//
//  Created by Michael Hills on 2/10/20.
//

#import "AppDelegate.h"
#if TARGET_OS_TV
#import "ViewController.h"
#endif
@import AVFoundation;

void rust_apple_main(void);


@interface AppDelegate ()

@end

@implementation AppDelegate



- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    // Override point for customization after application launch.

#if TARGET_OS_TV
    // tvOS requires programmatic window creation (no storyboards)
    self.window = [[UIWindow alloc] initWithFrame:[[UIScreen mainScreen] bounds]];
    self.window.rootViewController = [[ViewController alloc] init];
    [self.window makeKeyAndVisible];
#endif

    NSError *error;
    BOOL success;

    // It is necessary to access the sharedInstance so that calls to AudioSessionGetProperty
    // will work.
    AVAudioSession *session = AVAudioSession.sharedInstance;

#if TARGET_OS_TV
    // tvOS only supports Playback category (no microphone input, no DefaultToSpeaker)
    success = [session setCategory:AVAudioSessionCategoryPlayback error:&error];
#else
    // iOS: Setting up the category is not necessary, but generally advised.
    // Since this demo records and plays, lets use AVAudioSessionCategoryPlayAndRecord.
    // Also default to speaker as defaulting to the phone earpiece would be unusual.
    success = [session setCategory:AVAudioSessionCategoryPlayAndRecord
                       withOptions:AVAudioSessionCategoryOptionDefaultToSpeaker
                             error:&error];
#endif

    if (success) {
        NSLog(@"Calling rust_apple_main()");
        rust_apple_main();
    } else {
        NSLog(@"Failed to configure audio session category: %@", error);
    }

    return YES;
}

@end
